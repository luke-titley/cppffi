all:
	g++ -std=c++11 --shared -fPIC -I../ -I../imath_install/include Imath_c.cpp -o libImath_c.so
	gcc -I../ ../imath_install/lib/libImath.dylib libImath_c.so test_vec2f.cpp -o test_vec2f
	install_name_tool -add_rpath ../imath_install/lib/ test_vec2f
	./test_vec2f
