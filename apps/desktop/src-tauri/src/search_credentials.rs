use reqwest::blocking::Client;
use serde::Serialize;
use std::process::Command;
use std::time::Duration;

const SERVICE_NAME: &str = "wireless-charging-research-workbench.search";
const USER_AGENT: &str = "wireless-charging-research-workbench/0.10";

#[derive(Debug, Clone, Copy)]
struct Provider {
    id: &'static str,
    label: &'static str,
    description: &'static str,
    env_name: Option<&'static str>,
}

const PROVIDERS: [Provider; 4] = [
    Provider {
        id: "arxiv",
        label: "arXiv",
        description: "公开预印本检索，无需 API Key",
        env_name: None,
    },
    Provider {
        id: "openalex",
        label: "OpenAlex",
        description: "开放学术元数据与引用索引",
        env_name: Some("OPENALEX_API_KEY"),
    },
    Provider {
        id: "tavily",
        label: "Tavily",
        description: "面向学术站点的补充网页检索",
        env_name: Some("TAVILY_API_KEY"),
    },
    Provider {
        id: "serpapi",
        label: "Google Scholar（SerpApi）",
        description: "通过 SerpApi 查询 Google Scholar",
        env_name: Some("SERPAPI_API_KEY"),
    },
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchProviderStatus {
    pub id: String,
    pub label: String,
    pub description: String,
    pub requires_key: bool,
    pub configured: bool,
}

trait CredentialStore {
    fn get(&self, provider: &str) -> Result<Option<String>, String>;
    fn set(&self, provider: &str, value: &str) -> Result<(), String>;
    fn delete(&self, provider: &str) -> Result<(), String>;
}

struct SystemCredentialStore;

impl SystemCredentialStore {
    fn entry(provider: &str) -> Result<keyring::Entry, String> {
        keyring::Entry::new(SERVICE_NAME, provider)
            .map_err(|_| "Windows 凭据管理器初始化失败".to_string())
    }
}

impl CredentialStore for SystemCredentialStore {
    fn get(&self, provider: &str) -> Result<Option<String>, String> {
        match Self::entry(provider)?.get_password() {
            Ok(value) => Ok((!value.trim().is_empty()).then_some(value)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(_) => Err(format!("读取 {provider} 安全凭据失败")),
        }
    }

    fn set(&self, provider: &str, value: &str) -> Result<(), String> {
        Self::entry(provider)?
            .set_password(value)
            .map_err(|_| format!("保存 {provider} 安全凭据失败"))
    }

    fn delete(&self, provider: &str) -> Result<(), String> {
        match Self::entry(provider)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(_) => Err(format!("清除 {provider} 安全凭据失败")),
        }
    }
}

fn provider(id: &str) -> Result<Provider, String> {
    PROVIDERS
        .iter()
        .copied()
        .find(|item| item.id == id)
        .ok_or_else(|| "不支持的论文搜索服务".to_string())
}

fn statuses_with(store: &impl CredentialStore) -> Result<Vec<SearchProviderStatus>, String> {
    PROVIDERS
        .iter()
        .map(|item| {
            let configured = match item.env_name {
                Some(_) => store.get(item.id)?.is_some(),
                None => true,
            };
            Ok(SearchProviderStatus {
                id: item.id.to_string(),
                label: item.label.to_string(),
                description: item.description.to_string(),
                requires_key: item.env_name.is_some(),
                configured,
            })
        })
        .collect()
}

pub fn list_statuses() -> Result<Vec<SearchProviderStatus>, String> {
    statuses_with(&SystemCredentialStore)
}

pub fn save_key(provider_id: &str, key: &str) -> Result<SearchProviderStatus, String> {
    let item = provider(provider_id)?;
    if item.env_name.is_none() {
        return Err("该检索源不需要 API Key".to_string());
    }
    let key = key.trim();
    if key.is_empty() {
        return Err("API Key 不能为空；清除已有 Key 请使用清除按钮".to_string());
    }
    if key.len() > 4096 || key.chars().any(char::is_control) {
        return Err("API Key 格式无效".to_string());
    }
    let store = SystemCredentialStore;
    store.set(item.id, key)?;
    Ok(SearchProviderStatus {
        id: item.id.to_string(),
        label: item.label.to_string(),
        description: item.description.to_string(),
        requires_key: true,
        configured: true,
    })
}

pub fn delete_key(provider_id: &str) -> Result<SearchProviderStatus, String> {
    let item = provider(provider_id)?;
    if item.env_name.is_none() {
        return Err("该检索源不需要 API Key".to_string());
    }
    SystemCredentialStore.delete(item.id)?;
    Ok(SearchProviderStatus {
        id: item.id.to_string(),
        label: item.label.to_string(),
        description: item.description.to_string(),
        requires_key: true,
        configured: false,
    })
}

fn credential_environment_with(
    store: &impl CredentialStore,
) -> Result<Vec<(&'static str, String)>, String> {
    let mut values = Vec::new();
    for item in PROVIDERS {
        if let (Some(env_name), Some(value)) = (item.env_name, store.get(item.id)?) {
            values.push((env_name, value));
        }
    }
    Ok(values)
}

pub fn apply_to_command(command: &mut Command) -> Result<(), String> {
    for (name, value) in credential_environment_with(&SystemCredentialStore)? {
        command.env(name, value);
    }
    Ok(())
}

pub fn test_provider(provider_id: &str) -> Result<String, String> {
    let item = provider(provider_id)?;
    let store = SystemCredentialStore;
    let key = match item.env_name {
        Some(_) => store
            .get(item.id)?
            .ok_or_else(|| format!("{} 尚未配置 API Key", item.label))?,
        None => String::new(),
    };
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent(USER_AGENT)
        .build()
        .map_err(|_| "创建连接测试客户端失败".to_string())?;
    let response = match item.id {
        "arxiv" => client
            .get("https://export.arxiv.org/api/query")
            .query(&[
                ("search_query", "all:wireless charging"),
                ("start", "0"),
                ("max_results", "1"),
            ])
            .send(),
        "openalex" => client
            .get("https://api.openalex.org/works")
            .query(&[
                ("search", "wireless charging"),
                ("per-page", "1"),
                ("api_key", key.as_str()),
            ])
            .send(),
        "tavily" => client
            .post("https://api.tavily.com/search")
            .bearer_auth(&key)
            .json(&serde_json::json!({
                "query": "wireless charging scheduling",
                "max_results": 1,
                "include_answer": false,
                "include_raw_content": false,
                "include_images": false
            }))
            .send(),
        "serpapi" => client
            .get("https://serpapi.com/search.json")
            .query(&[
                ("engine", "google_scholar"),
                ("q", "wireless charging scheduling"),
                ("num", "1"),
                ("api_key", key.as_str()),
            ])
            .send(),
        _ => unreachable!(),
    }
    .map_err(|_| format!("{} 连接测试失败", item.label))?;
    if response.status().is_success() {
        Ok(format!("{} 连接成功", item.label))
    } else {
        Err(format!(
            "{} 返回 HTTP {}",
            item.label,
            response.status().as_u16()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[derive(Default)]
    struct MemoryStore(RefCell<HashMap<String, String>>);

    impl CredentialStore for MemoryStore {
        fn get(&self, provider: &str) -> Result<Option<String>, String> {
            Ok(self.0.borrow().get(provider).cloned())
        }

        fn set(&self, provider: &str, value: &str) -> Result<(), String> {
            self.0
                .borrow_mut()
                .insert(provider.to_string(), value.to_string());
            Ok(())
        }

        fn delete(&self, provider: &str) -> Result<(), String> {
            self.0.borrow_mut().remove(provider);
            Ok(())
        }
    }

    #[test]
    fn statuses_never_expose_stored_secret() {
        let store = MemoryStore::default();
        store.set("tavily", "top-secret-value").unwrap();
        let statuses = statuses_with(&store).unwrap();
        let tavily = statuses.iter().find(|item| item.id == "tavily").unwrap();
        assert!(tavily.configured);
        let json = serde_json::to_string(&statuses).unwrap();
        assert!(!json.contains("top-secret-value"));
        assert!(
            statuses
                .iter()
                .find(|item| item.id == "arxiv")
                .unwrap()
                .configured
        );
    }

    #[test]
    fn credential_environment_contains_only_configured_secret_providers() {
        let store = MemoryStore::default();
        store.set("openalex", "openalex-secret").unwrap();
        store.set("serpapi", "serp-secret").unwrap();
        let environment = credential_environment_with(&store).unwrap();
        assert_eq!(environment.len(), 2);
        assert!(environment.contains(&("OPENALEX_API_KEY", "openalex-secret".into())));
        assert!(environment.contains(&("SERPAPI_API_KEY", "serp-secret".into())));
        assert!(!environment
            .iter()
            .any(|(name, _)| *name == "TAVILY_API_KEY"));
    }

    #[test]
    fn provider_allowlist_rejects_unknown_ids() {
        assert_eq!(provider("unknown").unwrap_err(), "不支持的论文搜索服务");
    }
}
