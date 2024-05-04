#include <stdio.h>

#include <stdlib.h>

#include <windows.h>

int main(){

while(1){

//Verifica se o botão esquerdo do mouse está pressionado

if ((GetKeyState(VK_LBUTTON) & 0x80) != 0)//

{

Sleep(200);

printf( "Botao da esquerda apertado\n");

}

//Verifica se o botão direito do mouse está pressionado

if ((GetKeyState(VK_RBUTTON) & 0x80) != 0)

{

Sleep(200);

printf( "Botao da direita apertado\n");

//Verifica se a tecla A está pressionada

if ((GetKeyState(65) & 0x80) != 0) // 64 em decimal = 0x41 em hexadecimal

{

Sleep(200);

printf( "Tecla A pressionada\n");

}

}

return 0;
}
}
