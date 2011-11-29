import stdlib.themes.bootstrap
import stdlib.web.client
import stdlib.crypto

type pixel = {
  data : intmap(string);
  secret : string;
}

type upload_info = {
  id : string;
  secret : string;
  offset : int;
}

db /pixels : stringmap(pixel)

@client upload_data():void = (
  data:string = Dom.get_value(#img_data)
  length:int = String.length(data)
  piece_length:float = Int.to_float(length) / 100.0
  upload_data_aux(data, length, piece_length, {id="" secret="" offset=0})
)

@async @client rec upload_data_aux(data:string, length:int, piece_length:float, info:upload_info):void = (
  // connect to server and send first piece of data
  if info.offset<100 then
    do Dom.set_text(#progress, "{info.offset}%")
    o:int = Int.of_float(Int.to_float(info.offset) * piece_length)
    e:int = Int.of_float(Int.to_float(info.offset+1) * piece_length)
    l:int =
      if e > length then length-o
      else e-o
    next_info:upload_info =
      if l>0 then
        piece:string = String.substr(o, l, data)
        if info.id == "" then upload_first_piece(info, piece)
        else upload_next_piece(info, piece)
      else
        {id=info.id secret=info.secret offset=info.offset+1}
    upload_data_aux(data, length, piece_length, next_info)
  else
    do Dom.set_text(#progress, "100%")
    Client.goto("/{info.id}")
)

@server @publish upload_first_piece(info:upload_info, piece:string):upload_info = (
  // TODO: handle collision!
  // TODO: increase range of characters
  id:string = Random.string(4)
  secret:string = Random.string(10)
  data:intmap = Map.empty
  data = Map.add(0, piece, data)
  do /pixels[id] <- {~data ~secret}
  {~id ~secret offset=info.offset+1}
)

@server @publish upload_next_piece(info:upload_info, piece:string):upload_info = (
  pixel:pixel = /pixels[info.id]
  do if (pixel.secret == info.secret) then
    /pixels[info.id]/data[info.offset] <- piece
  else
    Debug.warning("secret mismatch: {info.secret} != {pixel.secret}")
  {id=info.id secret=info.secret offset=info.offset+1}
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

display_raw_image(id:string):resource = (
  p = /pixels[id]

  data:string = Map.fold((k, v, r -> String.concat("", [r, v])), p.data, "")

  // data is in the following format:
  // data:image/<png|jpeg|etc.>;base64,<base64 encoded data>
  // for now, we'll only locate ";base64," and ignore the first part
  // we'll tell the browser the image is image/png, even if that's
  // not the case (browsers are smart enough to figure things out)
  offset:int = Option.get(String.index(";base64,", data)) + 8
  data = String.sub(offset, String.length(data)-offset, data)
  data = Crypto.Base64.decode(data)
  Resource.raw_response(data, "image/png", {success})
)

display_image(id:string):resource = (
  p =
    if (Db.exists(@/pixels[id])) then
      /pixels[id]
    else
      // TODO: implement
      /pixels["4oh4"]

  display(
    <>
      <img class="preview" src="/img/{id}"/>
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
        <div id=#progress/>
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
  | {path={hd="img" tl={~hd ...} ...} ...} -> display_raw_image(hd)
  | {path={~hd ...} ...} -> display_image(hd)
)

server = Server.of_bundle([@static_include_directory("resources")])
server = Server.simple_dispatch(start)
