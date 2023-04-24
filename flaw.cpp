#include <iostream>
#include <string>
using namespace std;
string name = "Hello";
int integer =10;
class ADT {   
 public:    
    void print() { 
        cout << "name is:" << name <<endl;
        // we can even change the variable in outside.
        integer =11;
        cout <<"Integer is: "<<integer<<"\n";
        }
    void ddelete(){
        cout<<"The string is: "<<name<<endl;
        name.clear();
    }
    
  public:             
    int num;        
    string String;
    bool Bool;
   int getInt(){
   return num;
   }
   
   string getString(){
   return String;
   }
   
   bool getBool(){
   return Bool;
   }
};

int main() {
  ADT obj; 
     obj.print();
     obj.ddelete();
  cout<<"The string after the class method is: "<<name<<endl;     
     
  cout <<"The int is " << obj.getInt() << "\n";
  cout <<"The string is " << obj.getString() << "\n";
  cout <<"The Bool is " << obj.getBool() << "\n";
}
