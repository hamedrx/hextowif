# hextowif
Rust tool for converting Bitcoin HEX private key to WIF private key

You can't import Bitcoin HEX private key into any wallet directly. So you have to convert it to WIF (Wallet Import Format) private key. this code in RUST is for you.


# Free Code
This code is free of charge. 


# Download and build
You must have Rust and Cargo installed, then download and Build with Cargo:

```
git clone https://github.com/hamedrx/hextowif.git
cd hextowif
cargo build --release
```

# Usage
Go to release folder and run:

```
cd target/release
./hextowif
```

# Result:
Enter your HEX private key and get WIF keys:

```
Enter hex private key: f99455077e175b129cfad4cd6d3202d1542faa35cf97b0a23ae62d0b576d2ffa
Uncompressed WIF: 5KiCkcxU6Lt8Q7XPz47pdv1Zjcdr9r2SoqA7VrjiuXAujVKh24E
Compressed WIF:   L5arsXhpaakff1oASpm7FvyVV6Hdx3PNuhZDgNXZxpAjk6YHMv8m
```
 Now you can import these WIF keys into any Bitcoin wallet.
 
# Donations

BTC: 3Hamed9zYHQ9bwJACSidP52HjuPyZX67vf

Lightning Address: shivaz@speed.app

