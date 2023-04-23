#include <iostream>
#include <string>
using namespace std;

class ADT {      
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
  cout <<"The int is " << obj.getInt() << "\n";
  cout <<"The string is " << obj.getString() << "\n";
  cout <<"The Bool is " << obj.getBool() << "\n";
}
