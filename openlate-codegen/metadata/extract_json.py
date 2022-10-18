from pathlib import Path
import json
import importlib


def main():

    p = Path('.')
    dirs = [p / x for x in p.iterdir() if x.is_dir()]

    source = [list(dir.glob('**/*.py')) for dir in dirs]
    for src in source:
        for data in src:
            mod, js = data.parts
            mod = importlib.import_module(mod)
            for key, item in mod.metadata.items():
                f = open(str(data.parent / key) + '.json', "w")
                d = json.dump(item, f, indent=2)


if __name__ == "__main__":
    main()