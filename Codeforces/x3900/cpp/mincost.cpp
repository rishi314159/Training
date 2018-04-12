#include<iostream>

using namespace std;


long long find_min_cost(long long n,long long k,long long a,long long b,long long acc) {

   long long rem = n % k;

   if (rem != 0 and n >= k )
      return find_min_cost(n-rem, k, a, b, acc + a*rem);

   long long quo = n/k;

   if ( quo > 0 and quo * a + b < n* a)
      return find_min_cost(quo, k, a, b, acc + b);

   return (n-1)*a + acc;
}







int main() {
   long long n,k,a,b;
   cin >> n >> k >> a >> b;
   cout << find_min_cost(n,k,a,b,0) <<endl;
}

