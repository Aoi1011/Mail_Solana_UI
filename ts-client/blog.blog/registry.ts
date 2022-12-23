import { GeneratedType } from "@cosmjs/proto-signing";
import { MsgDeleteComment } from "./types/blog/blog/tx";
import { MsgCreatePost } from "./types/blog/blog/tx";
import { MsgCreateComment } from "./types/blog/blog/tx";

const msgTypes: Array<[string, GeneratedType]>  = [
    ["/blog.blog.MsgDeleteComment", MsgDeleteComment],
    ["/blog.blog.MsgCreatePost", MsgCreatePost],
    ["/blog.blog.MsgCreateComment", MsgCreateComment],
    
];

export { msgTypes }