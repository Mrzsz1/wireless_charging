import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path


SPEC = importlib.util.spec_from_file_location('wiki_lint', Path(__file__).parents[1] / 'tools' / 'wiki_lint.py')
wiki_lint = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = wiki_lint
SPEC.loader.exec_module(wiki_lint)


class WikiLintTests(unittest.TestCase):
    def test_relative_wikilink_resolves(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            source = root / 'wiki' / 'maps' / 'map.md'
            target = root / 'wiki' / 'sources' / 'src-a.md'
            source.parent.mkdir(parents=True)
            target.parent.mkdir(parents=True)
            source.write_text('', encoding='utf-8')
            target.write_text('', encoding='utf-8')
            hits = wiki_lint.resolve_link(source, '../sources/src-a', {})
            self.assertEqual(hits, [target.resolve()])

    def test_stem_fallback_resolves(self):
        target = Path('wiki/sources/src-a.md')
        hits = wiki_lint.resolve_link(Path('wiki/maps/map.md'), 'src-a', {'src-a': [target]})
        self.assertEqual(hits, [target])

    def test_missing_link_is_empty(self):
        hits = wiki_lint.resolve_link(Path('wiki/maps/map.md'), '../sources/missing', {})
        self.assertEqual(hits, [])


if __name__ == '__main__':
    unittest.main()
