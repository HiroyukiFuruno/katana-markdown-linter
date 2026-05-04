import os
import sys
import tomllib

def verify_zed_extension(extension_path):
    if not os.path.exists(extension_path):
        print(f"Extension path not found: {extension_path}")
        sys.exit(1)

    extension_toml_path = os.path.join(extension_path, 'extension.toml')
    if not os.path.exists(extension_toml_path):
        print(f"extension.toml not found in {extension_path}")
        sys.exit(1)

    with open(extension_toml_path, 'rb') as extension_toml:
        try:
            config = tomllib.load(extension_toml)
        except Exception as error:
            print(f"Failed to parse extension.toml: {error}")
            sys.exit(1)

    print(f"Verifying Zed extension: {config.get('name')}@{config.get('version')}")

    required_fields = ['id', 'name', 'version', 'schema_version', 'description']
    for field in required_fields:
        if field not in config:
            print(f"Missing required field in extension.toml: {field}")
            sys.exit(1)

    if not isinstance(config['id'], str) or not config['id'].strip():
        print('extension.toml id must be a non-empty string')
        sys.exit(1)
    if not isinstance(config['name'], str) or not config['name'].strip():
        print('extension.toml name must be a non-empty string')
        sys.exit(1)
    if not isinstance(config['schema_version'], int):
        print('extension.toml schema_version must be an integer')
        sys.exit(1)
    if config.get('schema_version') < 1:
        print('extension.toml schema_version must be 1 or greater')
        sys.exit(1)

    if 'enabled' in config and not isinstance(config['enabled'], bool):
        print('extension.toml enabled must be a boolean when present')
        sys.exit(1)

    # Check for languages config
    languages_path = os.path.join(extension_path, 'languages')
    if not os.path.exists(languages_path):
        print(f"languages directory not found in {extension_path}")
        sys.exit(1)

    print("Zed extension verification passed.")

if __name__ == "__main__":
    target_dir = sys.argv[1] if len(sys.argv) > 1 else 'editors/zed'
    verify_zed_extension(target_dir)
