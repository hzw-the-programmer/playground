package api

import (
	"net/http"
	"encoding/json"

	"github.com/google/uuid"
	"github.com/gorilla/mux"
)

type Item struct {
	ID uuid.UUID `json:"id"`
	Name string `json:"name"`
}

type Server struct {
	*mux.Router
	shoppingItems []Item
}

func NewServer() *Server {
	s := &Server{
		Router: mux.NewRouter(),
		shoppingItems: []Item{},
	}
	s.routes()
	return s
}

func (s *Server) routes() {
	s.Handle("/shopping-items", s.createShoppingItem()).Methods("POST")
	s.Handle("/shopping-items", s.listShoppingItems()).Methods("GET")
	s.Handle("/shopping-items/{id}", s.removeShoppingItem()).Methods("DELETE")
}

func (s *Server) createShoppingItem() appHandler {
	return func(w http.ResponseWriter, r *http.Request) *appError {
		var i Item
		if err := json.NewDecoder(r.Body).Decode(&i); err != nil {
			return &appError{err, err.Error(), http.StatusBadRequest}
		}

		i.ID = uuid.New()
		s.shoppingItems = append(s.shoppingItems, i)

		w.Header().Set("Content-Type", "application-json")
		if err := json.NewEncoder(w).Encode(i); err != nil {
			return &appError{err, err.Error(), http.StatusInternalServerError}
		}

		return nil
	}
}

func (s *Server) listShoppingItems() appHandler {
	return func(w http.ResponseWriter, r *http.Request) *appError {
		w.Header().Set("Content-Type", "application/json")
		if err := json.NewEncoder(w).Encode(s.shoppingItems); err != nil {
			return &appError{err, err.Error(), http.StatusInternalServerError}
		}

		return nil
	}
}

func (s *Server) removeShoppingItem() appHandler {
	return func(w http.ResponseWriter, r *http.Request) *appError {
		idStr, _ := mux.Vars(r)["id"]
		id, err := uuid.Parse(idStr)
		if err != nil {
			return &appError{err, err.Error(), http.StatusBadRequest}
		}

		for i, item := range s.shoppingItems {
			if item.ID == id {
				s.shoppingItems = append(s.shoppingItems[:i], s.shoppingItems[i+1:]...)
				break
			}
		}

		return nil
	}
}