all:
	g++ -I../ -I../imath_install/include Imath_c.cpp -o Imath_c.o
	g++ -I../ ../imath_install/lib/libImath.dylib Imath_c.o main.cpp -o tests
	install_name_tool -add_rpath ../imath_install/lib/ tests
