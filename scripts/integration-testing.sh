#!/bin/sh

start_http_server() {
  nohup cargo run --package cmd --bin http-server -- --conf=./config/http-server.toml > /dev/null 2>&1 &
      sleep 3
      while ! ps aux | grep -v grep | grep "http-server" > /dev/null; do
          echo "Process http-server has not started yet, wait 1s...."
          sleep 1
      done
      echo "Process http-server starts successfully and starts running the test case"
}

stop_http_server(){
    pc_no=`ps aux | grep -v grep | grep "http-server" | awk '{print $2}'`
    echo "http-server pid num: $pc_no"
    kill -2 "$pc_no"
    sleep 3

    while ps aux | grep -v grep | grep "http-server" > /dev/null; do
        echo "Process stopping http-server"
        sleep 1
    done
    echo "Process http-server stopped successfully"
}

start_http_server

cargo test

if [ $? -ne 0 ]; then
    echo "Test case failed to run"
    stop_http_server
    exit 1
else
    echo "Test case runs successfully"
    stop_http_server
fi
