# -*- coding: utf-8 -*-

from socket import if_nameindex
import tiktoken


def gpt_num_tokens(messages, model='gpt-3.5-turbo'):
    encoding = tiktoken.encoding_for_model(model)
    return len(encoding.encode(messages))

if __name__ == '__main__':
    s = '你是一名专业的视频内容编辑，帮我用简体中文总结视频的内容精华'
    print(gpt_num_tokens(s))