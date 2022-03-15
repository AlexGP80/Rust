#include <iostream>
using namespace std;
void fuckit(int x)
{
  cout << x << endl;
  x++;
  fuckit(x);
}

int main()
{
  fuckit(1);
}
