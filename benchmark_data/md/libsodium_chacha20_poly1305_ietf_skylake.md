[//]: # (@generated)

| operation     | cycles              |
|---------------|---------------------|
| encrypt_empty |   675.6789 ± 3.7884 |
| aad empty     |   675.1100 ± 4.0448 |
| decrypt empty |   703.9322 ± 4.1439 |
| encrypt empty |   673.4028 ± 3.4164 |

| bytes   | aad                 | decrypt             | encrypt             |
|---------|---------------------|---------------------|---------------------|
| 16b     |    44.3037 ± 0.2829 |    76.5289 ± 0.2581 |    74.2937 ± 0.4162 |
| 32b     |    22.9417 ± 0.0790 |    38.8931 ± 0.1407 |    38.1030 ± 0.1639 |
| 48b     |    16.2760 ± 0.1801 |    27.0530 ± 0.1318 |    26.9017 ± 0.3593 |
| 64b     |    12.4966 ± 0.0370 |    20.2695 ± 0.0471 |    19.7119 ± 0.0515 |
| 80b     |    10.4031 ± 0.0237 |    22.1326 ± 0.0978 |    21.6821 ± 0.0488 |
| 96b     |     9.0110 ± 0.0256 |    18.7804 ± 0.0914 |    18.4251 ± 0.0383 |
| 112b    |     8.1265 ± 0.0417 |    16.9154 ± 0.0988 |    16.4120 ± 0.0897 |
| 128b    |     7.4156 ± 0.0495 |    14.7610 ± 0.0940 |    14.5152 ± 0.0898 |
| 144b    |     6.8939 ± 0.0871 |    16.6205 ± 0.1056 |    16.3127 ± 0.1161 |
| 160b    |     6.3327 ± 0.0519 |    14.9244 ± 0.0910 |    14.6861 ± 0.0898 |
| 176b    |     5.8972 ± 0.0363 |    13.8064 ± 0.0456 |    13.6138 ± 0.0859 |
| 192b    |     5.5845 ± 0.0471 |    12.6374 ± 0.0621 |    12.5376 ± 0.1081 |
| 208b    |     5.2639 ± 0.0150 |    13.8640 ± 0.0439 |    13.7637 ± 0.0733 |
| 224b    |     5.0401 ± 0.0111 |    13.0779 ± 0.0453 |    12.8875 ± 0.0294 |
| 240b    |     4.8380 ± 0.0173 |    12.3702 ± 0.0280 |    12.2463 ± 0.0316 |
| 256b    |     4.6956 ± 0.0402 |    11.5822 ± 0.0295 |    11.4863 ± 0.0396 |
| 384b    |     3.8099 ± 0.0117 |    10.6335 ± 0.0359 |    10.5309 ± 0.0539 |
| 512b    |     3.3702 ± 0.0127 |    10.1767 ± 0.0308 |    10.0766 ± 0.0234 |
| 640b    |     3.1137 ± 0.0196 |     9.8811 ± 0.0649 |     9.8090 ± 0.0289 |
| 768b    |     2.9558 ± 0.0177 |     9.7597 ± 0.0448 |     9.7481 ± 0.0711 |
| 896b    |     2.8290 ± 0.0138 |     9.5354 ± 0.0178 |     9.5532 ± 0.0252 |
| 1kb     |     2.7102 ± 0.0081 |     9.4478 ± 0.0392 |     9.4102 ± 0.0227 |
| 1.125kb |     2.6371 ± 0.0082 |     9.3555 ± 0.0192 |     9.3655 ± 0.0472 |
| 1.25kb  |     2.5768 ± 0.0055 |     9.2854 ± 0.0168 |     9.2653 ± 0.0261 |
| 1.375kb |     2.5368 ± 0.0092 |     9.2275 ± 0.0196 |     9.2111 ± 0.0291 |
| 1.5kb   |     2.5237 ± 0.0128 |     9.2440 ± 0.0533 |     9.2024 ± 0.0223 |
| 1.625kb |     2.4800 ± 0.0081 |     9.1592 ± 0.0142 |     9.1740 ± 0.0314 |
| 1.75kb  |     2.4484 ± 0.0061 |     9.1429 ± 0.0265 |     9.1080 ± 0.0182 |
| 1.875kb |     2.4398 ± 0.0126 |     9.1871 ± 0.0536 |     9.1835 ± 0.0379 |
| 2kb     |     2.4133 ± 0.0091 |     9.1545 ± 0.0504 |     9.1134 ± 0.0536 |
| 2.125kb |     2.3968 ± 0.0113 |     9.0879 ± 0.0242 |     9.1017 ± 0.0394 |
| 2.25kb  |     2.3593 ± 0.0042 |     9.0421 ± 0.0188 |     9.0229 ± 0.0254 |
| 2.375kb |     2.3571 ± 0.0089 |     9.0451 ± 0.0141 |     9.0284 ± 0.0304 |
| 2.5kb   |     2.3516 ± 0.0062 |     9.0070 ± 0.0111 |     9.0034 ± 0.0225 |
| 2.625kb |     2.3390 ± 0.0063 |     9.0455 ± 0.0560 |     9.0027 ± 0.0333 |
| 2.75kb  |     2.3245 ± 0.0061 |     9.0038 ± 0.0432 |     8.9929 ± 0.0309 |
| 2.875kb |     2.3165 ± 0.0088 |     8.9899 ± 0.0209 |     8.9922 ± 0.0347 |
| 3kb     |     2.3163 ± 0.0083 |     9.0918 ± 0.0635 |     9.0634 ± 0.0589 |
| 3.125kb |     2.3006 ± 0.0081 |     8.9975 ± 0.0314 |     9.0131 ± 0.0486 |
| 3.25kb  |     2.2869 ± 0.0073 |     8.9442 ± 0.0264 |     9.0049 ± 0.0415 |
| 3.375kb |     2.2739 ± 0.0043 |     8.9630 ± 0.0253 |     8.9134 ± 0.0166 |
| 3.5kb   |     2.2631 ± 0.0041 |     8.9579 ± 0.0292 |     8.9095 ± 0.0201 |
| 3.625kb |     2.2562 ± 0.0081 |     8.9325 ± 0.0201 |     8.9193 ± 0.0198 |
| 3.75kb  |     2.2583 ± 0.0069 |     8.9179 ± 0.0189 |     8.9148 ± 0.0281 |
| 3.875kb |     2.2472 ± 0.0054 |     8.9440 ± 0.0158 |     8.8901 ± 0.0188 |
| 4kb     |     2.2394 ± 0.0047 |     8.9172 ± 0.0255 |     8.9122 ± 0.0478 |
| 16kb    |     2.1222 ± 0.0088 |     8.8426 ± 0.0500 |     8.8070 ± 0.0368 |
| 32kb    |     2.0926 ± 0.0070 |     8.7786 ± 0.0275 |     8.7654 ± 0.0273 |
| 48kb    |     2.0805 ± 0.0071 |     8.7309 ± 0.0407 |     8.7478 ± 0.0321 |
| 64kb    |     2.0723 ± 0.0043 |     8.7041 ± 0.0207 |     8.7057 ± 0.0197 |
| 80kb    |     2.0686 ± 0.0069 |     8.7044 ± 0.0205 |     8.6983 ± 0.0208 |
| 96kb    |     2.0716 ± 0.0072 |     8.6930 ± 0.0162 |     8.7196 ± 0.0266 |
| 112kb   |     2.0679 ± 0.0064 |     8.7455 ± 0.0379 |     8.7247 ± 0.0281 |
| 128kb   |     2.0701 ± 0.0068 |     8.7227 ± 0.0267 |     8.7236 ± 0.0312 |
| 256kb   |     2.0667 ± 0.0098 |     8.7205 ± 0.0424 |     8.7204 ± 0.0308 |
| 384kb   |     2.0670 ± 0.0065 |     8.6906 ± 0.0184 |     8.7046 ± 0.0232 |
| 512kb   |     2.0692 ± 0.0117 |     8.7048 ± 0.0277 |     8.7162 ± 0.0252 |
| 640kb   |     2.0599 ± 0.0052 |     8.7019 ± 0.0213 |     8.6871 ± 0.0185 |
| 768kb   |     2.0607 ± 0.0043 |     8.7226 ± 0.0346 |     8.7209 ± 0.0310 |
| 896kb   |     2.0616 ± 0.0072 |     8.7256 ± 0.0270 |     8.7193 ± 0.0277 |
| 1mb     |     2.0655 ± 0.0057 |     8.8023 ± 0.0312 |     8.7188 ± 0.0269 |

| metadata  |                                          |
|-----------|------------------------------------------|
| algorithm | chacha20_poly1305_ietf                   |
| cpu       | IntelSkylake (06_55H)                    |
| library   | libsodium                                |
| path      | libsodium_chacha20_poly1305_ietf_skylake |
| primitive | aead                                     |
| profile   | skylake                                  |
| version   | 1.0.19                                   |