
cd ..\..
cargo build
$env:AWS_ACCESS_KEY_ID='AKIARXNITSYZLQG3MRLY'
$env:AWS_SECRET_ACCESS_KEY='*************************************'
target\debug\aws_client.exe --region eu-west-2 --mode create-bucket --bucket testgilfe
target\debug\aws_client.exe --region eu-west-2 --mode delete-bucket --bucket testgilfe
cd examples\ps1