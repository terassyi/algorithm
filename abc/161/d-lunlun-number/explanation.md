# D: Lunlun Number
この問題は、Queue というデータ構造を用いることで効率的に解くことができます。まず、空の
Queue を 1 つ用意し、1, 2, ..., 9 を順に Enqueue します。それから、以下の操作を K 回行います。
• Queue に対して Dequeue を行う。取り出した要素を x とする。
• x mod 10 ̸= 0 なら、 10x + (x mod 10) − 1 を Enqueue する。
• 10x + (x mod 10) を Enqueue する。
• x mod 10 ̸= 9 なら、 10x + (x mod 10) + 1 を Enqueue する。
K 回目の操作において取り出した数が、 K 番目の Lunlun Number となっています。
