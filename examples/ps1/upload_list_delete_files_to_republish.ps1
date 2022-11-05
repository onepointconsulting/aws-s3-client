cd ..\..
cargo build
""
"** Upload to folder **"
"======================"
target\debug\aws_client.exe --region eu-central-1 --mode upload --bucket mdm-eu-prod-republish -g data\*.txt --target-folder folder_test_gil

""
"** List folder **"
"================="

target\debug\aws_client.exe --region eu-central-1 --mode list --bucket mdm-eu-prod-republish --list-regex-pattern ^.*folder_test_gil.+

""
"** Delete files in folder **"
"================="
target\debug\aws_client.exe --region eu-central-1 --mode delete --bucket mdm-eu-prod-republish --list-regex-pattern ^.*folder_test_gil.+
cd examples\ps1