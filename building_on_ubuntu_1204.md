### Building on Ubuntu 12.04

- install gcc-4.8
```
sudo add-apt-repository ppa:ubuntu-toolchain-r/test
sudo apt-get update; sudo apt-get install gcc-4.8 g++-4.8
sudo update-alternatives --remove-all gcc
sudo update-alternatives --remove-all g++
sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-4.8 20
sudo update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-4.8 20
sudo update-alternatives --config gcc
sudo update-alternatives --config g++
```

- install rocksdb
```
wget https://github.com/facebook/rocksdb/archive/rocksdb-3.8.tar.gz
tar xvf rocksdb-3.8.tar.gz && cd rocksdb-rocksdb-3.8 && make shared_lib
sudo make install
```

- install rust
```
curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
cargo install rustfmt
export PATH=/home/yunba/.cargo/bin:$PATH
cargo update
```

- make
```
cd tikv
make
```
