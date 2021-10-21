import os
import pathlib
import string
import hashlib
import base64


def list_files(base_dir):
    result = []
    for file in os.listdir(base_dir):
        current = os.path.join(base_dir, file)
        if os.path.isfile(os.path.join(base_dir, file)):
            result.append(current)
        else:
            result.extend(list_files(current))
    return result


if __name__ == '__main__':
    source_dir = "./resource"
    target_dir = "./src/resource"

    for source_file in list_files(source_dir):
        print(source_file)
        class_name = source_file
        idx = source_file.rfind('/')
        if 0 < idx:
            class_name = class_name[idx + 1:]
        class_name = string.capwords(class_name.replace("_", " ").replace(".", " ")).replace(" ", "")

        target_file = source_file.replace("./", "./src/")
        idx = target_file.rfind('.')
        if 0 < idx:
            target_file = target_file[0: idx] + ".rs"
        with open(source_file) as f:
            s = f.read()

        if os.path.exists(target_file):
            os.remove(target_file)

        hash = base64.urlsafe_b64encode(hashlib.md5(s.encode("utf-8")).digest()).decode("utf-8").replace("=", "")

        with open(target_file, mode='w') as f:
            f.write("""pub struct {0};

impl {0} {{
    pub const CONTENT: &'static str = r#\"""".format(class_name))
            f.write(s)
            f.write("""\
"#;

    #[allow(unused)]
    pub const ETAG: &'static str = "{0}";
}}
""".format(hash))
