type LineInfo = {
  file: string;
  line: number;
  col: number;
};
export function getLineInfo(index : number = 2) : LineInfo | undefined {
  let error = new Error();
  const stacks = error.stack.split("\n");
//   let i = 0;
//   for(i = 0; i < stacks.length; i++){
//     console.log(i, stacks[i])
//   }
  if(stacks.length <= index){
    console.log('fail to parse stack', stacks);
    return;
  }
  let caller_stack = stacks[index];
  //console.log(caller_stack);
  let start = caller_stack.indexOf('(');
  let end = caller_stack.lastIndexOf(')');
  if(start <0 ||  end < 0){
    console.log('fail to parse stack', caller_stack);
    return;
  }
  caller_stack = caller_stack.substring(start + 1, end);
  const [file,line, col] = caller_stack.split(':');

  const lineInfo : LineInfo = {
    file: file,
    line: parseInt(line),
    col: parseInt(col),
  };
  return lineInfo;

}

export function dot() {
  const lineInfo = getLineInfo(3);
  if(lineInfo){
    console.log(lineInfo.file, lineInfo.line, lineInfo.col);
  }
}

dot();