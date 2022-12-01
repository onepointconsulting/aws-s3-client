
cd ..\..
cargo build
target\debug\aws_client.exe --region eu-west-2 --mode copy-bucket-to-bucket --bucket boomi.gil.test --target-bucket tui.test --source-key e3bc44a9-cbfa-4152-85be-8511563f8b45 --target-key e3bc44a9-cbfa-4152-85be-8511563f8b45
cd examples\ps1