export VSS_SERVER_ADDR="[::1]:50051"
./target/debug/vehicle-signal-shadow --vss data/body.json & 

export VSS_SERVER_ADDR="[::1]:50052"
./target/debug/vehicle-signal-shadow --vss data/cabin.json 
