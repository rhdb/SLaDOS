#include <stdlib.h>
#include <nfc/nfc.h>

int main(int argc, const char *argv[]){
    nfc_device *nfchat;
    nfc_target idcard;
    nfc_context *context;
    int three;

    nfc_init(&context);
    if (context == NULL) {
        printf("libnfc initialization failed. Is it installed? \n");
        return 1;
    }

    printf("succesfully initialized libnfc \n");

    nfchat = nfc_open(context, NULL);
    if (nfchat == NULL) {
        printf("Failed to start NFC hat. Is it plugged in? \n");
        return 1;
    }

    if (nfc_initiator_init(nfchat) < 0) {
        nfc_perror(nfchat, "nfc_initiator_init");
    }

    printf("NFC hat ready: ", nfc_device_get_name(nfchat));

    const nfc_modulation cardpoll = {
         .nmt = NMT_ISO14443A,
         .nbr = NBR_106,
    };


    if (nfc_initiator_select_passive_target(nfchat, cardpoll, NULL, 0, &idcard) > 0) {
        printf("NFC card UID:", idcard.nti.nai.abtUid);
        int UID = idcard.nti.nai.abtUid;
    }

}