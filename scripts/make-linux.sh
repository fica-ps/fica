cd ../
g++ -shared -O3 -o out/fica.so src/capi.cpp src/fastica.cpp src/whitening.cpp src/contrast.cpp -I include -I eigen   
cd ./scripts
