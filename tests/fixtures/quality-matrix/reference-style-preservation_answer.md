# Reference and emphasis styles

Shortcut [*foo*] and collapsed [*foo*][] links must resolve, as must the corresponding images ![*foo*] and ![*foo*][]. An explicit reference [_foo_][id] and inline link [_bar_](/inline) may change emphasis style while keeping the link destination.

Strong shortcut [**strong**] and collapsed [**strong**][] links must resolve, as must the corresponding images ![**strong**] and ![**strong**][]. An explicit reference [__bold__][strong-id] and inline link [__weight__](/inline-strong) may change strong style while keeping the link destination.

Explicit reference images ![_alt_][image-id] and ![__alt__][strong-image-id] preserve their labels and destinations, as do inline images ![_alt_](/image-inline) and ![__alt__](/image-strong-inline).

[*foo*]: /shortcut
[**strong**]: /strong-shortcut
[id]: /explicit
[strong-id]: /strong-explicit
[image-id]: /image-reference
[strong-image-id]: /strong-image-reference
