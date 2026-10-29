# Reference and emphasis styles

Shortcut [*foo*] and collapsed [*foo*][] links must resolve, as must the corresponding images ![*foo*] and ![*foo*][]. An explicit reference [*foo*][id] and inline link [*bar*](/inline) may change emphasis style while keeping the link destination.

Strong shortcut [**strong**] and collapsed [**strong**][] links must resolve, as must the corresponding images ![**strong**] and ![**strong**][]. An explicit reference [**bold**][strong-id] and inline link [**weight**](/inline-strong) may change strong style while keeping the link destination.

Explicit reference images ![*alt*][image-id] and ![**alt**][strong-image-id] preserve their labels and destinations, as do inline images ![*alt*](/image-inline) and ![**alt**](/image-strong-inline).

[*foo*]: /shortcut
[**strong**]: /strong-shortcut
[id]: /explicit
[strong-id]: /strong-explicit
[image-id]: /image-reference
[strong-image-id]: /strong-image-reference
