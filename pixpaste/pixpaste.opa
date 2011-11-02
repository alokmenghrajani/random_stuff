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
  Client.go(Uri.of_absolute({Uri.default_absolute with
    schema = {some = "http"}
    domain = "localhost"
    port = {some = 8080}
    path = [Int.to_string(key)]}))
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

start(uri:Uri.relative):resource = (
  match uri
  | {path=[] ...} -> display(
      <>
        <div class="page-header"><h1>Just hit ctrl-v <small>to share an image</small></h1></div>
        <p>Hint: use cmd-control-shift-4 to copy a part of the screen to the clipboard</p>
        <input id=#img_data type="text" class="hidden"/>
        <div class="alert-message error" id=#error style="display: none"/>
        <div><img id=#preview class="preview" src="resources/preview.png"/></div>
        <div><input id=#btn type="button" class="btn" onclick={_ -> upload_data()} value="Upload"/></div>
        <script src="resources/ctrl_v.js"></script>
      </>
    )
  | {~path ... } -> display_image(List.head(path))
)

server = Server.of_bundle([@static_include_directory("resources")])
server = Server.simple_dispatch(start)
