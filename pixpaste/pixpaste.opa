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
  data:string = Option.get(Dom.get_property(#preview, "src"))
  length:int = String.length(data)
  piece_length:float = Int.to_float(length) / 20.0
  upload_data_aux(data, length, piece_length, {id="" secret="" offset=0})
)

@async @client rec upload_data_aux(data:string, length:int, piece_length:float, info:upload_info):void = (
  // connect to server and send first piece of data
  if info.offset<20 then
    do Dom.show(#progress)
    do Dom.set_value(#progress, "{info.offset}")
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
    ["resources/pixpaste.css"],
    <>
      {body}
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
      <div id="one">
        <img src="/img/{id}"/>
        <p>Link to your pixels: <a href="http://pixpaste.quaxio.com:8080/{id}">http://pixpaste.quaxio.com:8080/{id}</a></p>
      </div>
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
    | {some={environment={Macintosh} ...}} ->
      {instruction="Hit Command-V | Drag'n'Drop | Use the file uploader" hint="use Shift-Control-Command-4 to capture an area of your screen"}
    | _ ->
      {instruction="Hit Ctrl-V | Drag'n'Drop | Use the file uploader" hint="use Alt-PrtSc to capture the current window"}
  instruction:string = t.instruction;
  hint:string = t.hint;

  // PixlPaste. A simple, free & reliable way to share pixels.
  // pixels, images, photos, screenshots
  // share, upload, save, bin, cloud
  // paste, drag, drop, chooser, uploader

  display(
    <div id="outer"><div id="middle"><div id="inner">
      <div class="help">
        <div id="help1">paste from clipboard</div>
      </div>
      <div class="help">
        <div id="help2">drag 'n' drop an image</div>
      </div>
      <div class="help">
        <div id="help3">use a <a id="file_chooser">file chooser</a></div>
      </div>
      <div class="help">
        <span id="help4">
          <input id=#btn type="button" class="btn" onclick={_ -> upload_data()} value="share"/>
        </span>
      </div>
      <div class="help">
        <span id="help1_arrow"><img src="resources/1.png"/></span>
      </div>
      <div class="help">
        <span id="help2_arrow"><img src="resources/2.png"/></span>
      </div>
      <div class="help">
        <span id="help3_arrow"><img src="resources/3.png"/></span>
      </div>
      <div class="help">
        <span id="help4_arrow"><img src="resources/4.png"/></span>
      </div>
      <div id="outer"><div id="middle"><div id="inner">
        <div class="alert-message error" id=#error style="display: none"/>
        <img id=#preview class="preview" src="resources/preview.png"/>
        <progress id=#progress value="0" max="20" style="display: none"/>
      </div></div></div>
      <script src="resources/ctrl_v.js"></script>
    </div></div></div>
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
