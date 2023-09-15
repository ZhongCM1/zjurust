#!/bin/bash

# 运行服务器
echo "Starting server..."
cargo run --bin server &

# 等待服务器启动
sleep 5

# 运行客户端并执行 SET 操作
echo "Starting client and setting values..."
cargo run --bin client << EOF
set a 1
set b 2
exit
EOF

# 关闭服务器
echo "Stopping server..."
kill -SIGTERM %1

# 等待服务器进程结束
wait %1

# 重启服务器
echo "Restarting server..."
cargo run --bin server &

# 等待服务器启动
sleep 5

# 运行客户端，并执行 GET 操作
echo "Retrieving values from client..."
cargo run --bin client << EOF
get a
get b
EOF
