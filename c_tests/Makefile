all:
	g++ -std=c++11 --shared -fPIC -I../ -I../imath_install/include Imath_c.cpp -o libImath_c.so
	gcc -I../ ../imath_install/lib/libImath.dylib libImath_c.so main.cpp -o tests
	install_name_tool -add_rpath ../imath_install/lib/ tests
