#include<vector>
#include<iostream>
#include<algorithm>


using namespace std;


int find_min_set(vector<int>& v, int n, int d);

int main() {
   int n,d,tmp;
   vector<int> v;

   cin >> n >> d;
   for (int i=0;  i< n; i++) {
      cin >> tmp;
      v.push_back(tmp);
   }

   sort(v.begin(), v.end());

   cout << find_min_set(v,n,d) << endl;

   return 0;
}

int find_min_set(vector<int>& v, int n, int d) {
   int low = 0;
   int high = 0;
   int maxset = 0;

   for (low = 0 ; low < n; low++) {
      while (high < n and v[high] - v[low] <= d) {
         maxset = max(maxset, high - low +1);
         high++;
      }
   }

   return n-maxset;
}
