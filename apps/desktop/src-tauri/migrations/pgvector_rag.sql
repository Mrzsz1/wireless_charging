create extension if not exists vector;

create table if not exists public.rag_embeddings (
  repository_id text not null,
  snapshot_id text not null,
  block_id text not null,
  document_id text not null,
  kind text not null check (kind in ('wiki', 'paper', 'book')),
  granularity text not null check (granularity in ('document', 'section', 'semantic')),
  role text not null,
  model_id text not null,
  dimension integer not null check (dimension = 384),
  content_hash text not null,
  embedding vector(384) not null,
  active boolean not null default true,
  updated_at timestamptz not null default now(),
  primary key (repository_id, model_id, block_id)
);

create index if not exists rag_embeddings_lookup
  on public.rag_embeddings(repository_id, snapshot_id, model_id, kind, granularity, active);

create index if not exists rag_embeddings_hnsw
  on public.rag_embeddings using hnsw (embedding vector_cosine_ops);

create or replace function public.match_rag_embeddings(
  p_repository_id text,
  p_snapshot_id text,
  p_model_id text,
  p_query_embedding vector(384),
  p_match_count integer default 20,
  p_min_score double precision default null,
  p_kinds text[] default '{}',
  p_document_ids text[] default '{}',
  p_granularities text[] default '{}',
  p_roles text[] default '{}'
)
returns table(block_id text, score double precision)
language sql
stable
as $$
  select e.block_id, 1 - (e.embedding <=> p_query_embedding) as score
  from public.rag_embeddings e
  where e.repository_id = p_repository_id
    and e.snapshot_id = p_snapshot_id
    and e.model_id = p_model_id
    and e.active
    and (cardinality(p_kinds) = 0 or e.kind = any(p_kinds))
    and (cardinality(p_document_ids) = 0 or e.document_id = any(p_document_ids))
    and (cardinality(p_granularities) = 0 or e.granularity = any(p_granularities))
    and (cardinality(p_roles) = 0 or e.role = any(p_roles))
    and (p_min_score is null or 1 - (e.embedding <=> p_query_embedding) >= p_min_score)
  order by e.embedding <=> p_query_embedding, e.block_id
  limit greatest(1, least(p_match_count, 200));
$$;

create or replace function public.rag_embedding_stats(
  p_repository_id text,
  p_model_id text
)
returns table(vector_count bigint, document_count bigint)
language sql
stable
as $$
  select count(*), count(distinct document_id)
  from public.rag_embeddings
  where repository_id = p_repository_id and model_id = p_model_id and active;
$$;
