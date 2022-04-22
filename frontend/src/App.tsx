import { useAuth } from "./AuthenticationContext";
import { Route, Routes, useSearchParams, useNavigate } from "react-router-dom";
import { useEffect } from "react";

const App = () => {
  return (
    <Routes>
      <Route path="test/123" element={<Test />} />
      <Route path="auth" element={<Auth />} />
      <Route path="/" element={<Home />} />
    </Routes>
  );
};

const Home = () => {
  const authContext = useAuth();

  return (
    <div>
      <h1>Home!</h1>
      <h1>{authContext.logged_in ? "true" : "false"}</h1>
      <h1>{authContext.access_token}</h1>
      <h3>{authContext.client_id}</h3>
      <Login />
    </div>
  );
};

const Test = () => {
  return (
    <div>
      <h1>this is a test page!, get me back here!</h1>
      <Login />
    </div>
  );
};

const Auth = () => {
  const authContext = useAuth();
  let [searchParams] = useSearchParams();

  const code = searchParams.get("code");
  const route = searchParams.get("route");

  useEffect(() => {
    fetch(`/api/login?code=${code}`)
      .then((resp) => {
        if (resp.status <= 399) {
          return resp.text();
        }
      })
      .then((access_token) => {
        authContext.setAccessToken(access_token!);
        window.location.href = window.location.origin + route!;
      });
  }, []);

  return <div>{code}</div>;
};

const Login = () => {
  const authContext = useAuth();

  const login = async () => {
    authContext.login(authContext);
  };
  const logout = async () => {
    authContext.logout();
  };

  //if (authService.isPending()) {
  //  return <div>Loading...</div>;
  //}

  return (
    <div>
      <button onClick={login}>Login</button>
      <button onClick={logout}>Logout</button>
    </div>
  );
};

export default App;
