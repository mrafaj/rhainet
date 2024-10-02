from flask import Flask, send_file


app = Flask(__name__)


@app.route("/<string:page_name>", methods=["GET"])
def all_routes(page_name):
    return send_file(f"pages/{page_name}.rhai", mimetype="application/rhai")


if __name__ == "__main__":
    app.run()
