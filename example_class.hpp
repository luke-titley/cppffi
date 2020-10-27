//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
#define expose_c __attribute__((annotate("ffi_expose")))

class Example
{
public:
    Example(){}

    const char * getName() const
    {
        return "an_example_class";
    }

    expose_c
    const double & getAge() const
    {
        return m_age;
    }

private:
    double m_age;
};
