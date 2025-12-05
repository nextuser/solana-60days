    function toString(arr  :number[]):string{
        let str = String.fromCharCode(...arr);
        let index = str.indexOf("\x00");
        if(index != -1){
            str
        }
        return str;
    }
    console.log(toString([97, 98, 99, 33,0,0,0]));
