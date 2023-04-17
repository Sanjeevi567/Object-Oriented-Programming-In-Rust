#include <bits/stdc++.h>
using namespace std;
//This is defined outside the class but class able access it.
//which is not we want
string name = "Hello";
int integer =10;
class Geeks {
public:
    // Instance methods but access something outside than itself.
    void print() { 
        cout << "name is:" << name <<endl;
        // we can even change the variable in outside.
        integer =11;
        cout <<"Integer is: "<<integer;
        }
    void ddelete(){
        name.clear();
    }
};
int main()
{
    Geeks obj;
    obj.print();
    obj.ddelete();
    return 0;
}
