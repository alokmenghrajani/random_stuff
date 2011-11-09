import stdlib.themes.bootstrap
import stdlib.web.client

type pixel = {
  id : int;
  data : string;
}

db /pixels : intmap(pixel)

upload_data():void = (
  key = Db.fresh_key(@/pixels)
  do /pixels[key] <- {id=key; data=Dom.get_value(#img_data)}
  Client.goto("/{key}")
)

display(body):resource = (
  Resource.styled_page(
    "PixPaste",
    ["/resources/pixpaste.css"],
    <>
    <div class="topbar">
      <div class="topbar-inner">
        <div class="fill">
          <div class="container">
            <a class="brand" href="/">PixPaste</a>
          </div>
        </div>
      </div>
    </div>
    <div class="container">{body}</div>
    <br/>
    </>
  )
)

display_image(path:string):resource = (
  p = /pixels[Int.of_string(path)]
  display(
    <><img class="preview" src={p.data}/></>
  )
)

display_home():resource = (
  match HttpRequest.get_user_agent()
    | {some={renderer={~Gecko} ...}} -> display_pixpaste()
    | {some={renderer={~Webkit variant={~Chrome}} ...}} -> display_pixpaste()
    | _ -> display(
      <>
        <h1>Sorry, your browser is not supported</h1>
        <p>PixPaste is tested with the following browsers:</p>
        <ul>
          <li>Firefox 3+: works</li>
          <li>Chrome: works</li>
          <li>Safari: will not be supported</li>
          <li>Opera: will not be supported</li>
          <li>Internet Explorer: work in progress</li>
          <li>Others: ???</li>
        </ul>
      </>)
)

display_pixpaste():resource = (
  t = match HttpRequest.get_user_agent()
    | {some={environment={Macintosh} ...}} -> {instruction="Just hit Command-V" hint="use Shift-Control-Command-4 to capture an area of your screen"}
    | _ -> {instruction="Just hit Ctrl-V" hint="use Alt-PrtSc to capture the current window"}
  instruction:string = t.instruction;
  hint:string = t.hint;

  display(
    <>
      <section>
        <div class="page-header"><h1>{instruction} <small>to share an image</small></h1></div>
        <p>Hint: {hint}</p>
        <input id=#img_data type="text" class="hidden"/>
        <div class="alert-message error" id=#error style="display: none"/>
        <div><img id=#preview class="preview" src="resources/preview.png"/></div>
        <div><input id=#btn type="button" class="btn" onclick={_ -> upload_data()} value="Upload"/></div>
      </section>
      <script src="resources/ctrl_v.js"></script>
    </>
  )
)

start(uri:Uri.relative):resource = (
  match uri
  | {path=[] ...} -> display_home()
  | {~path ... } -> match List.head(path)
    | "favicon.ico" -> @static_resource("resources/favicon.png")
    | "favicon.gif" -> @static_resource("resources/favicon.png")
    | img -> display_image(img)
)

server = Server.of_bundle([@static_include_directory("resources")])
server = Server.simple_dispatch(start)
