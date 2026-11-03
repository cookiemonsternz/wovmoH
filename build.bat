cd ./wovmoh-rs
cargo build
cd ../
copy ./wovmoh-rs/target/debug/wovmoh-rs.exe ./wovmoh-gd/
copy ./wovmoh-rs/target/debug/wovmoh-rs.exe ./build/
cd wovmoh-gd
godot
