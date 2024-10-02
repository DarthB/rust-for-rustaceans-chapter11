// stdlib.h MUST BE INCLUDED for Windows to link the right malloc / free functions,
// see: https://stackoverflow.com/questions/33393633/access-violation-after-malloc
#include <stdlib.h>

// printf
#include <stdio.h>

void *init_context()
{
    void *reval = malloc(27);
    char *ptr = reval;
    for (int i = 0; i < 26; ++i, ++ptr)
    {
        *ptr = 65 + i;
    }
    ((char *)reval)[26] = '\0';

    printf("Init in C=%x, str=%s\n", reval, (char *)reval);
    return reval;
}

void free_context(void *context)
{
    printf("free_context in C=%x\n", context);
    free(context);
}

int context_is_lower(void *context)
{
    if (((char *)(context))[0] == 65)
        return 1;
    else
        return 0;
}

int context_is_upper(void *context)
{
    if (((char *)(context))[0] == 97)
        return 1;
    else
        return 0;
}

void context_to_lower(void *context)
{
    for (int i = 0; i < 26; ++i)
    {
        ((char *)(context))[i] += ' ';
    }

    printf("context_to_lower called: C=%x, str=%s\n", context, (char *)context);
}

void context_to_upper(void *context)
{
    for (int i = 0; i < 26; ++i)
    {
        ((char *)(context))[i] -= ' ';
    }

    printf("context_to_upper called: C=%x, str=%s\n", context, (char *)context);
}

void default_callback(const char *subobject)
{
    printf("default C callback\n");
}

typedef void (*subobject_created_callback_t)(const char *subobject);
subobject_created_callback_t g_created_callback = &default_callback;

int create_subobject(void *parent_context, int offset, void **subobject)
{
    printf("create_so with offset=%i from C\n", offset);
    if (offset < 0 || offset > 26)
        return -1;

    *subobject = (void *)((char *)parent_context + offset);
    g_created_callback(*subobject);
    return 0;
}

void set_callback(subobject_created_callback_t callback)
{
    g_created_callback = callback;
}