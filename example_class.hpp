//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
class Example
{
public:
    Example(){}

    const char * getName() const {
        return "an_example_class";
    }

    const double & getAge() const {
        return m_age;
    }

private:
    double m_age;
};
