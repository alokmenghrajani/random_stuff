import stdlib.themes.bootstrap
import stdlib.web.client
import stdlib.crypto

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
    <footer class="footer">
      <div class="container">
        <p>Designed and built using <a href="http://www.opalang.org">opa</a>.
          Code available on <a href="http://github.com/alokmenghrajani/random_stuff/tree/master/pixpaste">github.com</a>.<br/>
          <a href="http://goo.gl/mod/G7PK">Suggest improvements!</a>
        </p>
      </div>
    </footer>
    </>
  )
)

display_raw_image(id:int):resource = (
  p = /pixels[id]

  // TODO: fix this ugly hack :(
  data = String.sub(22, String.length(p.data)-22, p.data)
  data = Crypto.Base64.decode(data)
  Resource.raw_response(data, "image/png", {success})
)

display_image(id:int):resource = (
  p = /pixels[id]
  display(
    <>
      <img class="preview" src="/img/{p.id}"/>
      <p>Share this link: <a href="http://pixpaste.quaxio.com:8080/{id}">http://pixpaste.quaxio.com:8080/{id}</a></p>
    </>
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
  | {path={nil} ...} -> display_home()
  | {path={hd="favicon.ico" ...} ...} -> @static_resource("resources/favicon.png")
  | {path={hd="favicon.gif" ...} ...} -> @static_resource("resources/favicon.png")
  | {path={hd="img" tl={~hd ...} ...} ...} -> display_raw_image(Int.of_string(hd))
  | {path={~hd ...} ...} -> display_image(Int.of_string(hd))
)

server = Server.of_bundle([@static_include_directory("resources")])
server = Server.simple_dispatch(start)
