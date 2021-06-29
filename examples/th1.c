#include <stdio.h>

int main(){
    int contador, flag = 0;
    int input[] = {15, 36, 180};
    for(contador = 0; contador<3 && flag == 0; contador++){
        int n, cnt = 0, senhaInt, primeFlag = 0;
        n = input[contador];
        double senha = 0;
        while(n%2==0){
            cnt++;
            senha += (2.0/cnt);
            n/=2;
        }
        for(int i=3; i<=n; i+=2){
            while(n%i==0){
                cnt++;
                senha += ((double)i/cnt);
                n/=i;
            }
        }
        senhaInt = senha;
        if(senhaInt!=senha) senhaInt++;
        if(senhaInt%2==0) primeFlag = 1;
        for(int i=3; i*i<senhaInt && primeFlag == 0; i++) if(senhaInt%i==0) primeFlag = 1;
        if(primeFlag==0){
            printf("SHERLIRO SALVOU MULITTLE\n");
            flag = 1;
        }
        else printf("ERRO\n");
    }
    if(contador!=3) for(contador; contador<3; contador++) printf("SINAL OFF\n");

    return 0;
}