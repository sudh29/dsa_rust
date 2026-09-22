class Solution{
public:

	void rearrange(int arr[], int n) {
	    // code here

	    int arr1[n];
	    int arr2[n];
	    int k=0;
	    int j=0;
	    for(int i=0;i<n;i++){
	        if (arr[i]<0){
	            arr1[k]=arr[i];
	            k+=1;
	        }
	        else{
	            arr2[j]=arr[i];
	            j+=1;
	        }
	    }

        int i=0;
        if (k>=j){
            for(int m=0;m<j;m++){
                arr[i]=arr2[m];
                i+=1;
                arr[i]=arr1[m];
                i+=1;
            }
            for(int m=j;m<k;m++){
                arr[i]=arr1[m];
                i+=1;
            }
        }
        else{
            for(int m=0;m<k;m++){
                arr[i]=arr2[m];
                i+=1;
                arr[i]=arr1[m];
                i+=1;
            }
            for(int m=k;m<j;m++){
                arr[i]=arr2[m];
                i+=1;
            }
        }

	}
};
