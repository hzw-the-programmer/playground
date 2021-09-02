https://linux.die.net/man/3/strncpy

A simple implementation of strncpy() might be:

char *
strncpy(char *dest, const char *src, size_t n)
{
    size_t i;

   for (i = 0; i < n && src[i] != '\0'; i++)
        dest[i] = src[i];
    for ( ; i < n; i++)
        dest[i] = '\0';

   return dest;
}

If there is no terminating null byte in the first n bytes of src, strncpy() produces an unterminated string in dest. You can force termination using something like the following:

strncpy(buf, str, n);
if (n > 0)
    buf[n - 1]= '\0';
