cd ../
g++ -shared -O3 -o out/fica.dll src/capi.cpp src/fastica.cpp src/whitening.cpp src/contrast.cpp -I include -I eigen   
cd ./scripts
