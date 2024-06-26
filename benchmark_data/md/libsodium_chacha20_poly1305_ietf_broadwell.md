[//]: # (@generated)

| operation     | cycles              |
|---------------|---------------------|
| encrypt_empty |   698.0022 ± 4.0340 |
| aad empty     |   696.1776 ± 3.0060 |
| decrypt empty |   723.7278 ± 2.9821 |
| encrypt empty |   696.4708 ± 2.8973 |

| bytes   | aad                 | decrypt             | encrypt             |
|---------|---------------------|---------------------|---------------------|
| 16b     |    45.8571 ± 0.1802 |    79.1182 ± 0.3604 |    76.8297 ± 0.2969 |
| 32b     |    23.7050 ± 0.1463 |    40.2544 ± 0.1383 |    39.7388 ± 0.2062 |
| 48b     |    16.3555 ± 0.0727 |    27.6708 ± 0.1193 |    26.8774 ± 0.1028 |
| 64b     |    12.8137 ± 0.0439 |    20.8778 ± 0.0926 |    20.3120 ± 0.0726 |
| 80b     |    10.6432 ± 0.0224 |    22.6368 ± 0.0805 |    22.2397 ± 0.0633 |
| 96b     |     9.2012 ± 0.0150 |    19.2840 ± 0.0758 |    18.9564 ± 0.0300 |
| 112b    |     8.2034 ± 0.0324 |    16.8811 ± 0.0605 |    16.5663 ± 0.0531 |
| 128b    |     7.4060 ± 0.0229 |    14.7949 ± 0.0230 |    14.5563 ± 0.0383 |
| 144b    |     6.8592 ± 0.0387 |    16.5038 ± 0.0220 |    16.2791 ± 0.0309 |
| 160b    |     6.3344 ± 0.0244 |    15.0749 ± 0.0355 |    14.8837 ± 0.0241 |
| 176b    |     5.9305 ± 0.0164 |    13.9619 ± 0.0444 |    13.7224 ± 0.0301 |
| 192b    |     5.6057 ± 0.0155 |    12.8239 ± 0.0477 |    12.6397 ± 0.0325 |
| 208b    |     5.3173 ± 0.0122 |    14.1658 ± 0.0324 |    13.9856 ± 0.0351 |
| 224b    |     5.0982 ± 0.0152 |    13.2860 ± 0.0366 |    13.1613 ± 0.0312 |
| 240b    |     4.8912 ± 0.0157 |    12.5322 ± 0.0160 |    12.4102 ± 0.0186 |
| 256b    |     4.7312 ± 0.0209 |    11.8227 ± 0.0238 |    11.7056 ± 0.0376 |
| 384b    |     3.8041 ± 0.0084 |    10.8169 ± 0.0210 |    10.7316 ± 0.0240 |
| 512b    |     3.3613 ± 0.0104 |    10.3617 ± 0.0497 |    10.2938 ± 0.0388 |
| 640b    |     3.0945 ± 0.0131 |    10.0250 ± 0.0271 |     9.9939 ± 0.0363 |
| 768b    |     2.9020 ± 0.0048 |     9.8779 ± 0.0295 |     9.8082 ± 0.0232 |
| 896b    |     2.7801 ± 0.0088 |     9.7757 ± 0.0402 |     9.7174 ± 0.0390 |
| 1kb     |     2.6966 ± 0.0107 |     9.6750 ± 0.0324 |     9.6386 ± 0.0539 |
| 1.125kb |     2.6156 ± 0.0099 |     9.5422 ± 0.0381 |     9.5020 ± 0.0242 |
| 1.25kb  |     2.5440 ± 0.0047 |     9.4788 ± 0.0358 |     9.4785 ± 0.0499 |
| 1.375kb |     2.5022 ± 0.0103 |     9.4155 ± 0.0398 |     9.3729 ± 0.0166 |
| 1.5kb   |     2.4850 ± 0.0073 |     9.3755 ± 0.0456 |     9.3419 ± 0.0186 |
| 1.625kb |     2.4498 ± 0.0094 |     9.3311 ± 0.0241 |     9.3361 ± 0.0460 |
| 1.75kb  |     2.4105 ± 0.0042 |     9.3201 ± 0.0323 |     9.2730 ± 0.0114 |
| 1.875kb |     2.3893 ± 0.0086 |     9.2270 ± 0.0088 |     9.2285 ± 0.0173 |
| 2kb     |     2.3676 ± 0.0060 |     9.2236 ± 0.0242 |     9.1889 ± 0.0083 |
| 2.125kb |     2.3354 ± 0.0029 |     9.2043 ± 0.0244 |     9.1984 ± 0.0090 |
| 2.25kb  |     2.3167 ± 0.0053 |     9.1642 ± 0.0209 |     9.1608 ± 0.0221 |
| 2.375kb |     2.3165 ± 0.0054 |     9.1596 ± 0.0182 |     9.1613 ± 0.0134 |
| 2.5kb   |     2.3147 ± 0.0056 |     9.1680 ± 0.0270 |     9.1555 ± 0.0181 |
| 2.625kb |     2.3057 ± 0.0072 |     9.1588 ± 0.0140 |     9.1363 ± 0.0170 |
| 2.75kb  |     2.2887 ± 0.0054 |     9.1300 ± 0.0142 |     9.1160 ± 0.0174 |
| 2.875kb |     2.2794 ± 0.0045 |     9.1319 ± 0.0147 |     9.1085 ± 0.0251 |
| 3kb     |     2.2637 ± 0.0051 |     9.1118 ± 0.0410 |     9.0979 ± 0.0157 |
| 3.125kb |     2.2517 ± 0.0049 |     9.0768 ± 0.0087 |     9.0927 ± 0.0160 |
| 3.25kb  |     2.2452 ± 0.0048 |     9.0901 ± 0.0128 |     9.0768 ± 0.0147 |
| 3.375kb |     2.2542 ± 0.0073 |     9.0835 ± 0.0244 |     9.1005 ± 0.0444 |
| 3.5kb   |     2.2301 ± 0.0039 |     9.0794 ± 0.0218 |     9.0701 ± 0.0142 |
| 3.625kb |     2.2263 ± 0.0080 |     9.0688 ± 0.0222 |     9.0664 ± 0.0154 |
| 3.75kb  |     2.2044 ± 0.0030 |     9.0468 ± 0.0092 |     9.0449 ± 0.0151 |
| 3.875kb |     2.2120 ± 0.0063 |     9.0331 ± 0.0064 |     9.0253 ± 0.0091 |
| 4kb     |     2.1962 ± 0.0037 |     9.0609 ± 0.0179 |     9.0819 ± 0.0244 |
| 16kb    |     2.0621 ± 0.0067 |     8.9099 ± 0.0201 |     8.9087 ± 0.0289 |
| 32kb    |     2.0324 ± 0.0030 |     8.8567 ± 0.0267 |     8.8671 ± 0.0242 |
| 48kb    |     2.0232 ± 0.0048 |     8.8387 ± 0.0160 |     8.8529 ± 0.0192 |
| 64kb    |     2.0196 ± 0.0046 |     8.8595 ± 0.0186 |     8.8331 ± 0.0099 |
| 80kb    |     2.0263 ± 0.0068 |     8.8751 ± 0.0302 |     8.8570 ± 0.0275 |
| 96kb    |     2.0152 ± 0.0044 |     8.8490 ± 0.0234 |     8.8976 ± 0.0280 |
| 112kb   |     2.0287 ± 0.0056 |     8.8886 ± 0.0304 |     8.9122 ± 0.0278 |
| 128kb   |     2.0203 ± 0.0053 |     8.8340 ± 0.0144 |     8.8822 ± 0.0333 |
| 256kb   |     2.0134 ± 0.0056 |     8.8655 ± 0.0288 |     8.8281 ± 0.0139 |
| 384kb   |     2.0092 ± 0.0030 |     8.8310 ± 0.0188 |     8.8403 ± 0.0212 |
| 512kb   |     2.0089 ± 0.0032 |     8.8515 ± 0.0255 |     8.8494 ± 0.0290 |
| 640kb   |     2.0149 ± 0.0063 |     8.8604 ± 0.0366 |     8.8813 ± 0.0350 |
| 768kb   |     2.0731 ± 0.0562 |     8.8851 ± 0.0454 |     8.8578 ± 0.0299 |
| 896kb   |     2.0134 ± 0.0060 |     8.9249 ± 0.0250 |     8.8931 ± 0.0272 |
| 1mb     |     2.0121 ± 0.0055 |     8.8762 ± 0.0331 |     8.9053 ± 0.0319 |

| metadata  |                                            |
|-----------|--------------------------------------------|
| algorithm | chacha20_poly1305_ietf                     |
| cpu       | IntelBroadwell (06_3dH)                    |
| library   | libsodium                                  |
| path      | libsodium_chacha20_poly1305_ietf_broadwell |
| primitive | aead                                       |
| profile   | broadwell                                  |
| version   | 1.0.19                                     |
