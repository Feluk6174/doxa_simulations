#include <iostream>
#include <cmath>
using namespace std;

int get_n_connected(int n){
    int res = log2(n);
    return res;
}

int main(){
    cout << 0 << endl;
    int n = 4;
    int m = 0;
    for (int i=1; i<n-1; i++){
        m = m + i;
    }
    cout << 1 << endl;
    int n_connected = get_n_connected(n)*((n-2)-1);
    cout << n << " " << m << " " << n_connected << " " << get_n_connected(n) << endl;
    cout << 2 << endl;
    int start_array_len = pow(2.0, 1);
    int start_array[start_array_len][1] = {{0}, {1}};
    cout << 3 << endl;
    for(int i=1; i<m; i++){
        cout << "3.5" << endl;
        int end_array_len = pow(2.0, i+1);
        int end_array[end_array_len][i+1];
        cout << 4 << endl;
        for(int j=0; j<start_array_len; j++){
            for(int k=0; k<i; k++){
                end_array[2*j][k] = start_array[j][k];
                end_array[2*j+1][k] = start_array[j][k];
            }
            end_array[2*j][i+1] = 0;
            end_array[2*j+1][i+1] = 1;
            cout << 5 << endl;
        }
        cout << 6 << endl;
        int start_array_len = pow(2.0, i+1);
        int start_array[start_array_len][i+1];
        for(int j=0; j < start_array_len; j++){
            for(int k=0; k < i; k++){
                cout << 5 << endl;
                start_array[j][k] = end_array[j][k];
                cout << 5 << endl;
            }
        }
        cout << "end " << i << endl;
    }
}