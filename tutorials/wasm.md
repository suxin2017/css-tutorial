# wasm 使用

rust 使用 wasm

具体教程可以看这篇文章[https://rustwasm.github.io/docs/wasm-pack/](https://rustwasm.github.io/docs/wasm-pack/)

```
wasm-pack build
```

```
.gitignore
css_tutorial.d.ts
css_tutorial.js
css_tutorial_bg.js
css_tutorial_bg.wasm
css_tutorial_bg.wasm.d.ts
package.json
README.md
```

我们通过 script 引入下

```js
<script type="module">import init, {parse} from "./pkg/css_tutorial.js"</script>
```

<style>
    .content{
        overflow-y: visible;
    }
    .token {
        display: inline-block;
        padding: 4px;
        border: 1px solid #666;
        margin: 2px;
        border-radius: 4px;
    }
</style>

<div style="position: sticky; 
    top: 64px;backgroun:#fff;">
    <textarea rows="20" style="width:100%;" id="input" autofocus>
table col[class*=col-] {
    position: static;
    display: table-column;
    float: none
}
</textarea>
    <button id="generate">generate</button>
</div>
<div id="tag" style="width:100%;min-height:200px;font-size:14px;"></div>

<script type="module">
    import init, { parse } from "./pkg/css_tutorial.js"
        const t = document.getElementById("tag");

    init().then(() => {
        let code = "";
        const input = document.getElementById("input");
        const g = document.getElementById("generate");
        input.onkeydown = (e)=>{
            e.stopPropagation()
        }
        const dfs = (node) => {
            if(node){
                    let { node_type, range } = node;
            let printStr = `[${node_type}] ${code.substring(range.start_pos, range.end_pos)}`;

            let start = `<ul><li>`
            let tag = `<div class="token" style="color:var(--${colorRandom()})" data-start=${range.start_pos
                } data-end=${range.end_pos}>${printStr}</div>`
            let children = ""
            if (node.children) {
                children = node.children.map(n => {
                    return dfs(n)
            }).join('');
            }
            let end = `</li></ul>`
            return `${start}${tag}${children}${end}`
            }
            return "";
        }

        function run() {
            t.innerHTML = "";
            code = input.value;
            try{
                console.time("parse")
                 let result = parse(code)
                 console.timeEnd("parse")
            result = JSON.parse(result);
            let d = dfs(result.root)
            t.innerHTML = d; 
            }catch(e){
                console.error(e);
                   t.innerHTML = `<pre>${e.message}\n${e.stack}</pre>`;

            }

        }
        g.onclick = run;
        run();

        function colorRandom() {
            const color = [
                "yellow",
                "orange",
                "red",
                "magenta",
                "violet",
                "blue",
                "cyan",
                "green",
            ];
            return color[Math.floor(Math.random() * color.length)];
        }
        function gt(token) {
            let { node_type, range } = token;
            let printStr = `[${node_type}] ${code.substring(range.start_pos, range.end_pos)}`;
            return `<div class="token" style="color:var(--${colorRandom()})" data-start=${range.start_pos
                } data-end=${range.end_pos}>${printStr}</div>`;
        }

        t.addEventListener(
            "mouseover",
            function (event) {
                event.target.style.borderColor = "orange";
                let start = event.target.dataset["start"];
                let end = event.target.dataset["end"];
                setTimeout(() => {
                    input.focus();
                    input.setSelectionRange(start, end);
                }, 10);

                event.preventDefault();
            },
            false

        );
        t.addEventListener(
            "mouseout",
            function (event) {
                event.target.style.borderColor = "";
            },
            false

        );
    }).catch((e)=>{
        console.error(e);
        t.innerHTML = `<pre>${e.message}\n${e.stack}</pre>`;

    })

</script>

好啦我们的教程到此结束了，下次再见。
