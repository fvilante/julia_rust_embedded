
//TODO: Make this text available for external module use (public) through 'use' keyword and not 'include' keyword

use avr_progmem::progmem;

progmem! {
    static progmem string E0 = "Erro de carga de parametro";
    static progmem string S0 = "Posicao Inicial";
    static progmem string S1 = "Posicao Final";
    static progmem string S2 = "Velocidade de Avanco";
    static progmem string S3 = "Velocidade de Retorno";
    static progmem string S4 = "Aceleracao de Avanco";
    static progmem string S5 = "Aceleracao de Retorno";
    static progmem string S6 = "Start Automatico no Avanco";
    static progmem string S7 = "Start Automatico no Retorno";

    //NOTE: it is possible to load any type in progmem not only strings
    static progmem A0: [u8; 6] = [0,1,2,3,4,5];
}
