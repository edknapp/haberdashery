[//]: # (@generated)

| operation     | cycles                |
|---------------|-----------------------|
| cipher_init   |  4977.7327 ± 328.8010 |
| decrypt_init  |   250.1101 ± 27.9168  |
| encrypt_init  |   252.3607 ± 27.7520  |
| key_init      |   273.1662 ± 26.7665  |
| aad empty     |   934.2861 ± 87.4099  |
| decrypt empty |  1251.0654 ± 160.0750 |
| encrypt empty |  1254.2033 ± 148.9511 |

| bytes   | aad                 | decrypt              | encrypt             |
|---------|---------------------|----------------------|---------------------|
| 16b     |    67.2863 ± 9.5947 |    97.0135 ± 11.2368 |    80.6414 ± 0.1632 |
| 32b     |    29.5825 ± 0.1003 |    42.0768 ± 0.1111  |    41.7283 ± 0.1716 |
| 48b     |    19.9664 ± 0.0409 |    28.5760 ± 0.0800  |    28.4252 ± 0.0877 |
| 64b     |    15.1351 ± 0.0380 |    20.6261 ± 0.0785  |    20.6631 ± 0.0626 |
| 80b     |    12.4413 ± 0.0299 |    21.6366 ± 0.0723  |    20.9157 ± 0.0381 |
| 96b     |    10.4776 ± 0.0479 |    18.2020 ± 0.0397  |    17.9260 ± 0.0426 |
| 112b    |     9.3124 ± 0.0233 |    15.9508 ± 0.0361  |    15.7969 ± 0.0300 |
| 128b    |     8.7254 ± 0.0224 |    12.6411 ± 0.0362  |    12.2218 ± 0.0213 |
| 144b    |     7.7935 ± 0.0247 |    13.6331 ± 0.0189  |    13.4710 ± 0.0375 |
| 160b    |     7.1839 ± 0.0229 |    12.5741 ± 0.0417  |    12.2939 ± 0.0228 |
| 176b    |     6.5948 ± 0.0253 |    11.5996 ± 0.0235  |    11.5169 ± 0.0302 |
| 192b    |     5.9831 ± 0.0252 |     9.5165 ± 0.0252  |     9.3212 ± 0.0161 |
| 208b    |     5.5774 ± 0.0240 |    10.6267 ± 0.0452  |    10.3948 ± 0.0290 |
| 224b    |     5.2346 ± 0.0205 |    10.0397 ± 0.0390  |     9.8512 ± 0.0286 |
| 240b    |     4.9313 ± 0.0117 |     9.5543 ± 0.0310  |     9.3446 ± 0.0254 |
| 256b    |     4.6310 ± 0.0060 |     7.2850 ± 0.0264  |     7.1334 ± 0.0148 |
| 384b    |     3.2602 ± 0.0120 |     5.0546 ± 0.0093  |     4.9599 ± 0.0128 |
| 512b    |     2.5679 ± 0.0084 |     3.9310 ± 0.0117  |     3.8373 ± 0.0177 |
| 640b    |     2.1728 ± 0.0088 |     4.1524 ± 0.0117  |     4.1035 ± 0.0169 |
| 768b    |     1.8935 ± 0.0054 |     3.5368 ± 0.0056  |     3.5163 ± 0.0086 |
| 896b    |     1.6957 ± 0.0036 |     3.1285 ± 0.0093  |     3.0836 ± 0.0114 |
| 1kb     |     1.5489 ± 0.0040 |     2.7985 ± 0.0030  |     2.7555 ± 0.0046 |
| 1.125kb |     1.4340 ± 0.0042 |     3.0538 ± 0.0078  |     3.0243 ± 0.0071 |
| 1.25kb  |     1.3472 ± 0.0043 |     2.8169 ± 0.0129  |     2.7797 ± 0.0092 |
| 1.375kb |     1.2721 ± 0.0049 |     2.6056 ± 0.0065  |     2.5877 ± 0.0098 |
| 1.5kb   |     1.2049 ± 0.0041 |     2.4263 ± 0.0067  |     2.4121 ± 0.0084 |
| 1.625kb |     1.1504 ± 0.0030 |     2.6396 ± 0.0115  |     2.6295 ± 0.0097 |
| 1.75kb  |     1.1122 ± 0.0038 |     2.4919 ± 0.0087  |     2.4760 ± 0.0158 |
| 1.875kb |     1.0673 ± 0.0038 |     2.3467 ± 0.0048  |     2.3391 ± 0.0046 |
| 2kb     |     1.0309 ± 0.0031 |     2.2407 ± 0.0092  |     2.2286 ± 0.0099 |
| 2.125kb |     0.9986 ± 0.0016 |     2.4080 ± 0.0120  |     2.3863 ± 0.0055 |
| 2.25kb  |     0.9743 ± 0.0039 |     2.3000 ± 0.0050  |     2.2826 ± 0.0061 |
| 2.375kb |     0.9473 ± 0.0017 |     2.2103 ± 0.0060  |     2.1944 ± 0.0058 |
| 2.5kb   |     0.9274 ± 0.0038 |     2.1261 ± 0.0064  |     2.1110 ± 0.0058 |
| 2.625kb |     0.9081 ± 0.0028 |     2.2738 ± 0.0060  |     2.2541 ± 0.0059 |
| 2.75kb  |     0.8876 ± 0.0010 |     2.2122 ± 0.0075  |     2.1820 ± 0.0072 |
| 2.875kb |     0.8709 ± 0.0019 |     2.1201 ± 0.0038  |     2.0998 ± 0.0047 |
| 3kb     |     0.8556 ± 0.0013 |     2.0506 ± 0.0048  |     2.0339 ± 0.0026 |
| 3.125kb |     0.8414 ± 0.0010 |     2.1672 ± 0.0058  |     2.1565 ± 0.0015 |
| 3.25kb  |     0.8312 ± 0.0031 |     2.1125 ± 0.0058  |     2.1089 ± 0.0056 |
| 3.375kb |     0.8180 ± 0.0014 |     2.0621 ± 0.0082  |     2.0366 ± 0.0061 |
| 3.5kb   |     0.8086 ± 0.0027 |     1.9974 ± 0.0045  |     1.9896 ± 0.0030 |
| 3.625kb |     0.7958 ± 0.0018 |     2.1091 ± 0.0040  |     2.0925 ± 0.0046 |
| 3.75kb  |     0.7859 ± 0.0012 |     2.0644 ± 0.0057  |     2.0423 ± 0.0045 |
| 3.875kb |     0.7803 ± 0.0030 |     2.0134 ± 0.0040  |     1.9983 ± 0.0037 |
| 4kb     |     0.7707 ± 0.0035 |     1.9701 ± 0.0065  |     1.9567 ± 0.0044 |
| 16kb    |     0.5775 ± 0.0022 |     1.7570 ± 0.0026  |     1.7798 ± 0.0032 |
| 32kb    |     0.5440 ± 0.0018 |     1.7182 ± 0.0033  |     1.7215 ± 0.0014 |
| 48kb    |     0.5353 ± 0.0008 |     1.7135 ± 0.0064  |     1.7136 ± 0.0025 |
| 64kb    |     0.5305 ± 0.0008 |     1.7033 ± 0.0041  |     1.7073 ± 0.0039 |
| 80kb    |     0.5261 ± 0.0011 |     1.6931 ± 0.0024  |     1.6977 ± 0.0028 |
| 96kb    |     0.5240 ± 0.0008 |     1.6916 ± 0.0030  |     1.6957 ± 0.0041 |
| 112kb   |     0.5221 ± 0.0030 |     1.6926 ± 0.0032  |     1.6939 ± 0.0024 |
| 128kb   |     0.5194 ± 0.0009 |     1.6919 ± 0.0029  |     1.6898 ± 0.0049 |
| 256kb   |     0.5145 ± 0.0007 |     1.6854 ± 0.0037  |     1.6831 ± 0.0027 |
| 384kb   |     0.5136 ± 0.0012 |     1.6890 ± 0.0064  |     1.6843 ± 0.0026 |
| 512kb   |     0.5142 ± 0.0014 |     1.6897 ± 0.0033  |     1.7005 ± 0.0044 |
| 640kb   |     0.5129 ± 0.0010 |     1.6908 ± 0.0034  |     1.7000 ± 0.0036 |
| 768kb   |     0.5136 ± 0.0009 |     1.7115 ± 0.0030  |     1.7164 ± 0.0035 |
| 896kb   |     0.5180 ± 0.0014 |     1.7182 ± 0.0052  |     1.7038 ± 0.0021 |
| 1mb     |     0.5153 ± 0.0022 |     1.6917 ± 0.0009  |     1.7250 ± 0.0082 |

| metadata  |                                     |
|-----------|-------------------------------------|
| algorithm | chacha20poly1305                    |
| cpu       | IntelBroadwell (06_3dH)             |
| library   | openssl                             |
| path      | openssl_chacha20_poly1305_broadwell |
| primitive | aead                                |
| profile   | broadwell                           |
| version   | 3.2.1                               |
