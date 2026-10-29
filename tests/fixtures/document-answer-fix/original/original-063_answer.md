# Inline code and escaped emphasis

The examples `\*literal\*` and `\_literal\_` are code spans and must remain literal. Outside code, an escaped asterisk \* is ordinary punctuation, while this _valid emphasis_ remains semantic emphasis. Keeping both forms in one reviewed document helps detect a scanner that lets a quoted marker affect its choice of style.

The note also contains `a_b_c`, a Unicode token naïve, and a bracketed label [overview](#overview). The output should preserve their boundaries and meaning exactly.

## Overview

This section defines the overview destination used above and keeps the sample link valid.
