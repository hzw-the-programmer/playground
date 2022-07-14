https://www.cs.utexas.edu/~mitra/honors/soln.html
Choose p = 3 and q = 11
Compute n = p * q = 3 * 11 = 33
Compute φ(n) = (p - 1) * (q - 1) = 2 * 10 = 20
Choose e such that 1 < e < φ(n) and e and φ (n) are coprime. Let e = 7
Compute a value for d such that (d * e) % φ(n) = 1. One solution is d = 3 [(3 * 7) % 20 = 1]
Public key is (e, n) => (7, 33)
Private key is (d, n) => (3, 33)
The encryption of m = 2 is c = 27 % 33 = 29
The decryption of c = 29 is m = 293 % 33 = 2

https://www.geeksforgeeks.org/rsa-algorithm-cryptography/

Generating Public Key :

Select two prime no's. Suppose P = 53 and Q = 59.
Now First part of the Public key  : n = P*Q = 3127.

 We also need a small exponent say e : 
But e Must be 

An integer.

Not be a factor of n.
 
1 < e < Φ(n) [Φ(n) is discussed below], 
Let us now consider it to be equal to 3.

    
Our Public Key is made of n and e

Generating Private Key :

We need to calculate Φ(n) :
Such that Φ(n) = (P-1)(Q-1)     
      so,  Φ(n) = 3016

    
Now calculate Private Key, d : 
d = (k*Φ(n) + 1) / e for some integer k
For k = 2, value of d is 2011.

```
// C program for RSA asymmetric cryptographic
// algorithm. For demonstration values are
// relatively small compared to practical
// application
#include<stdio.h>
#include<math.h>
  
// Returns gcd of a and b
int gcd(int a, int h)
{
    int temp;
    while (1)
    {
        temp = a%h;
        if (temp == 0)
          return h;
        a = h;
        h = temp;
    }
}
  
// Code to demonstrate RSA algorithm
int main()
{
    // Two random prime numbers
    double p = 3;
    double q = 7;
  
    // First part of public key:
    double n = p*q;
  
    // Finding other part of public key.
    // e stands for encrypt
    double e = 2;
    double phi = (p-1)*(q-1);
    while (e < phi)
    {
        // e must be co-prime to phi and
        // smaller than phi.
        if (gcd(e, phi)==1)
            break;
        else
            e++;
    }
  
    // Private key (d stands for decrypt)
    // choosing d such that it satisfies
    // d*e = 1 + k * totient
    int k = 2;  // A constant value
    double d = (1 + (k*phi))/e;
  
    // Message to be encrypted
    double msg = 20;
  
    printf("Message data = %lf", msg);
  
    // Encryption c = (msg ^ e) % n
    double c = pow(msg, e);
    c = fmod(c, n);
    printf("\nEncrypted data = %lf", c);
  
    // Decryption m = (c ^ d) % n
    double m = pow(c, d);
    m = fmod(m, n);
    printf("\nOriginal Message Sent = %lf", m);
  
    return 0;
}
```
