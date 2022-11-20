# Код TOTP в командной строке

Консольная утилита для получения актуального кода [TOTP](https://ru.wikipedia.org/wiki/Time-based_One-time_Password_Algorithm) по URL.

Как использовать:

```test
Usage: totp_cli <URL> [COMMAND]

Commands:
never   
always  
ansi    
auto    
help    Print this message or the help of the given subcommand(s)

Arguments:
<URL>

Options:
-h, --help     Print help information
-V, --version  Print version information
```

Команда (`[COMMAND]`) опциональна и позволяет раскрашивать или не раскрашивать текстовый вывод утилиты. По умолчанию
раскрашивание происходит автоматически.

Пример запуска утилиты:

```shell
totp_cli "otpauth://totp/GitHub:constantoine@github.com?secret=KRSXG5CTMVRXEZLUKN2XAZLSKNSWG4TFOQ&issuer=GitHub"
```
